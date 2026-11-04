extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut val: i8) {
    let empty_s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            <std_basic_string_char__std_char_traits_char___std_allocator_char__>::default(),
        ));
    let short_s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { 3_u64 },
                { val },
                None,
            ),
        ));
    let long_s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { 1100_u64 },
                { val },
                None,
            ),
        ));
    ({
        let ___lhs: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            empty_s.as_pointer();
        let ___rhs: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            empty_s.as_pointer();
        swap_1(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            short_s.as_pointer();
        let ___rhs: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            short_s.as_pointer();
        swap_1(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            long_s.as_pointer();
        let ___rhs: Ptr<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
            long_s.as_pointer();
        swap_1(___lhs, ___rhs)
    });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_1(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_1(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_1(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_1(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_1(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_1(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { 11100_u64 },
                { val },
                None,
            ),
        ));
    ({ swap_1(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_1(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { 111_u64 },
                { val },
                None,
            ),
        ));
    ({ swap_1(long_s.as_pointer(), long_s3.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_2(mut val: i32) {
    let empty_s: Value<
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
    > = Rc::new(RefCell::new(
        <std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__>::default(),
    ));
    let short_s: Value<
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { 3_u64 },
            { val },
            None,
        ),
    ));
    let long_s: Value<std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__> =
        Rc::new(RefCell::new(
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
                { 1100_u64 },
                { val },
                None,
            ),
        ));
    ({
        let ___lhs: Ptr<
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = empty_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = empty_s.as_pointer();
        swap_3(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = short_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = short_s.as_pointer();
        swap_3(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = long_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
        > = long_s.as_pointer();
        swap_3(___lhs, ___rhs)
    });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_3(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_3(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_3(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_3(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_3(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_3(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { 11100_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_3(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_3(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { 111_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_3(long_s.as_pointer(), long_s3.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_4(mut val: u16) {
    let empty_s: Value<
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
    > = Rc::new(RefCell::new(
        <std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__>::default(),
    ));
    let short_s: Value<
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 3_u64 },
            { val },
            None,
        ),
    ));
    let long_s: Value<
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 1100_u64 },
            { val },
            None,
        ),
    ));
    ({
        let ___lhs: Ptr<
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
        > = empty_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
        > = empty_s.as_pointer();
        swap_5(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
        > = short_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
        > = short_s.as_pointer();
        swap_5(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
        > = long_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
        > = long_s.as_pointer();
        swap_5(___lhs, ___rhs)
    });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_5(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_5(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_5(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_5(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_5(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_5(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 11100_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_5(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_5(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 111_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_5(long_s.as_pointer(), long_s3.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_6(mut val: u32) {
    let empty_s: Value<
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
    > = Rc::new(RefCell::new(
        <std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__>::default(),
    ));
    let short_s: Value<
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 3_u64 },
            { val },
            None,
        ),
    ));
    let long_s: Value<
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 1100_u64 },
            { val },
            None,
        ),
    ));
    ({
        let ___lhs: Ptr<
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
        > = empty_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
        > = empty_s.as_pointer();
        swap_7(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
        > = short_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
        > = short_s.as_pointer();
        swap_7(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
        > = long_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
        > = long_s.as_pointer();
        swap_7(___lhs, ___rhs)
    });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_7(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_7(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_7(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_7(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_7(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_7(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 11100_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_7(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_7(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 111_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_7(long_s.as_pointer(), long_s3.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_8(mut val: u8) {
    let empty_s: Value<
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
    > = Rc::new(RefCell::new(
        <std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__>::default(),
    ));
    let short_s: Value<
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
            { 3_u64 },
            { val },
            None,
        ),
    ));
    let long_s: Value<std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__> =
        Rc::new(RefCell::new(
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
                { 1100_u64 },
                { val },
                None,
            ),
        ));
    ({
        let ___lhs: Ptr<
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
        > = empty_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
        > = empty_s.as_pointer();
        swap_9(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
        > = short_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
        > = short_s.as_pointer();
        swap_9(___lhs, ___rhs)
    });
    ({
        let ___lhs: Ptr<
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
        > = long_s.as_pointer();
        let ___rhs: Ptr<
            std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
        > = long_s.as_pointer();
        swap_9(___lhs, ___rhs)
    });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_9(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_9(empty_s.as_pointer(), short_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_9(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_9(empty_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_9(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_9(short_s.as_pointer(), long_s.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
            { 11100_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_9(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({ swap_9(long_s.as_pointer(), long_s2.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__,
    > = Rc::new(RefCell::new(
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
            { 111_u64 },
            { val },
            None,
        ),
    ));
    ({ swap_9(long_s.as_pointer(), long_s3.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
    ({ test_0(('x' as i8)) });
    ({ test_2((120 as i32)) });
    ({ test_4((120 as u16)) });
    ({ test_6((120 as u32)) });
    ({ test_8((120 as u8)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
