extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    ({ test_0(<std_basic_string_view_char__std_char_traits_char__>::default()) });
    ({ test_1(<std_basic_string_view_char16_t__std_char_traits_char16_t__>::default()) });
    ({ test_2(<std_basic_string_view_char32_t__std_char_traits_char32_t__>::default()) });
    ({
        test_0(std_basic_string_view_char__std_char_traits_char__::new_1({
            Ptr::<i8>::from_string_literal(b"123")
        }))
    });
    ({
        test_3(
            std_basic_string_view_char8_t__std_char_traits_char8_t__::new_1({
                Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8])
            }),
        )
    });
    ({
        test_1(
            std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1({
                Ptr::<u16>::from_string_literal(&[49 as u16, 50 as u16, 51 as u16, 0 as u16])
            }),
        )
    });
    ({
        test_2(
            std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1({
                Ptr::<u32>::from_string_literal(&[49 as u32, 50 as u32, 51 as u32, 0 as u32])
            }),
        )
    });
    ({ test_4(<std_basic_string_view_wchar_t__std_char_traits_wchar_t__>::default()) });
    ({
        test_4(
            std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1({
                Ptr::<i32>::from_string_literal(&[49 as i32, 50 as i32, 51 as i32, 0 as i32])
            }),
        )
    });
    {
        let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
            std_basic_string_view_char__std_char_traits_char__::new_2(
                { Ptr::<i8>::from_string_literal(b"123") },
                { 3_u64 },
            ),
        ));
        let u8sv: Value<std_basic_string_view_char8_t__std_char_traits_char8_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char8_t__std_char_traits_char8_t__::new_2(
                    { Ptr::<u8>::from_string_literal(&[49 as u8, 50 as u8, 51 as u8, 0 as u8]) },
                    { 3_u64 },
                ),
            ));
        let u16sv: Value<std_basic_string_view_char16_t__std_char_traits_char16_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char16_t__std_char_traits_char16_t__::new_2(
                    {
                        Ptr::<u16>::from_string_literal(&[
                            49 as u16, 50 as u16, 51 as u16, 0 as u16,
                        ])
                    },
                    { 3_u64 },
                ),
            ));
        let u32sv: Value<std_basic_string_view_char32_t__std_char_traits_char32_t__> =
            Rc::new(RefCell::new(
                std_basic_string_view_char32_t__std_char_traits_char32_t__::new_2(
                    {
                        Ptr::<u32>::from_string_literal(&[
                            49 as u32, 50 as u32, 51 as u32, 0 as u32,
                        ])
                    },
                    { 3_u64 },
                ),
            ));
        const _: () = assert!(true, "*sv.begin() == sv[0]");;
        const _: () = assert!(true, "*u8sv.begin() == u8sv[0]");;
        const _: () = assert!(true, "*u16sv.begin() == u16sv[0]");;
        const _: () = assert!(true, "*u32sv.begin() == u32sv[0]");;
        const _: () = assert!(true, "*sv.cbegin() == sv[0]");;
        const _: () = assert!(true, "*u8sv.cbegin() == u8sv[0]");;
        const _: () = assert!(true, "*u16sv.cbegin() == u16sv[0]");;
        const _: () = assert!(true, "*u32sv.cbegin() == u32sv[0]");;
        {
            let wsv: Value<std_basic_string_view_wchar_t__std_char_traits_wchar_t__> =
                Rc::new(RefCell::new(
                    std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_2(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                49 as i32, 50 as i32, 51 as i32, 0 as i32,
                            ])
                        },
                        { 3_u64 },
                    ),
                ));
            const _: () = assert!(true, "*wsv.begin() == wsv[0]");;
            const _: () = assert!(true, "*wsv.cbegin() == wsv[0]");;
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
