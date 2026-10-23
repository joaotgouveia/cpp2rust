extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_string_asan_correct_0(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_1(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_2(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_3(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_4(_a0: Ptr<()>) -> () {
    return true;
}
pub fn test_5(mut val: ()) {
    let empty_s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let short_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { 3_() },
            { val },
            None,
        ),
    ));
    let long_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { 1100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let long_s2: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { 11100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut long_s3: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { 111_() },
        { val },
        None,
    );
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_6(mut val: ()) {
    let empty_s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let short_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { 3_() },
            { val },
            None,
        ),
    ));
    let long_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { 1100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let long_s2: Value<()> = Rc::new(RefCell::new(
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { 11100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut long_s3: () =
        std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
            { 111_() },
            { val },
            None,
        );
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_7(mut val: ()) {
    let empty_s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let short_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 3_() },
            { val },
            None,
        ),
    ));
    let long_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 1100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let long_s2: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 11100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut long_s3: () =
        std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t__::new_1(
            { 111_() },
            { val },
            None,
        );
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_8(mut val: ()) {
    let empty_s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let short_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 3_() },
            { val },
            None,
        ),
    ));
    let long_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 1100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let long_s2: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 11100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut long_s3: () =
        std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t__::new_1(
            { 111_() },
            { val },
            None,
        );
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_9(mut val: ()) {
    let empty_s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let short_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
            { 3_() },
            { val },
            None,
        ),
    ));
    let long_s: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
            { 1100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let long_s2: Value<()> = Rc::new(RefCell::new(
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
            { 11100_() },
            { val },
            None,
        ),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut long_s3: () =
        std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t__::new_1(
            { 111_() },
            { val },
            None,
        );
    unimplemented!();
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
    ({ test_5(('x' as ())) });
    ({ test_6((120 as ())) });
    ({ test_7((120 as ())) });
    ({ test_8((120 as ())) });
    ({ test_9((120 as ())) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
