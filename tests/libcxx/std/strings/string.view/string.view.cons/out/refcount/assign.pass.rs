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
    ({
        cpp2rust_assert_fail_0(
            ({
                test_1(std_basic_string_view_char__std_char_traits_char__::new_1({
                    Ptr::<i8>::from_string_literal(b"1234")
                }))
            }),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ({
                test_2(
                    std_basic_string_view_char8_t__std_char_traits_char8_t__::new_1({
                        Ptr::<u8>::from_string_literal(&[
                            49 as u8, 50 as u8, 51 as u8, 52 as u8, 0 as u8,
                        ])
                    }),
                )
            }),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ({
                test_3(
                    std_basic_string_view_char16_t__std_char_traits_char16_t__::new_1({
                        Ptr::<u16>::from_string_literal(&[
                            49 as u16, 50 as u16, 51 as u16, 52 as u16, 0 as u16,
                        ])
                    }),
                )
            }),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ({
                test_4(
                    std_basic_string_view_char32_t__std_char_traits_char32_t__::new_1({
                        Ptr::<u32>::from_string_literal(&[
                            49 as u32, 50 as u32, 51 as u32, 52 as u32, 0 as u32,
                        ])
                    }),
                )
            }),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ({
                test_5(
                    std_basic_string_view_wchar_t__std_char_traits_wchar_t__::new_1({
                        Ptr::<i32>::from_string_literal(&[
                            49 as i32, 50 as i32, 51 as i32, 52 as i32, 0 as i32,
                        ])
                    }),
                )
            }),
        )
    });
    const _: () = assert!(true, "test<std::string_view>({{\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u8string_view>({{u8\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u16string_view>({{u\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::u32string_view>({{U\"abc\", 3}})");;
    const _: () = assert!(true, "test<std::wstring_view>({{L\"abc\", 3}})");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
