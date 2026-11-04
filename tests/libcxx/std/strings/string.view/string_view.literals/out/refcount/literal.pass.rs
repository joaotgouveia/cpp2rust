extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_char__std_char_traits_char____std_basic_string_view_char__std_char_traits_char___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_char__std_char_traits_char____std_basic_string_view_char__std_char_traits_char___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_char8_t__std_char_traits_char8_t____std_basic_string_view_char8_t__std_char_traits_char8_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_char8_t__std_char_traits_char8_t____std_basic_string_view_char8_t__std_char_traits_char8_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_wchar_t__std_char_traits_wchar_t____std_basic_string_view_wchar_t__std_char_traits_wchar_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_wchar_t__std_char_traits_wchar_t____std_basic_string_view_wchar_t__std_char_traits_wchar_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_char16_t__std_char_traits_char16_t____std_basic_string_view_char16_t__std_char_traits_char16_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_char16_t__std_char_traits_char16_t____std_basic_string_view_char16_t__std_char_traits_char16_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_view_char32_t__std_char_traits_char32_t____std_basic_string_view_char32_t__std_char_traits_char32_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_view_char32_t__std_char_traits_char32_t____std_basic_string_view_char32_t__std_char_traits_char32_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let foo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let Lfoo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let u8foo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ufoo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let Ufoo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!() == 0_()));
        unimplemented!();
        assert!((unimplemented!() == 0_()));
        unimplemented!();
        assert!((unimplemented!() == 0_()));
        unimplemented!();
        assert!((unimplemented!() == 0_()));
        unimplemented!();
        assert!((unimplemented!() == 0_()));
        unimplemented!();
        assert!((unimplemented!() == 1_()));
        unimplemented!();
        assert!((unimplemented!() == 1_()));
        unimplemented!();
        assert!((unimplemented!() == 1_()));
        unimplemented!();
        assert!((unimplemented!() == 1_()));
        unimplemented!();
        assert!((unimplemented!() == 1_()));
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        const _: () = assert!(true, "\"ABC\"sv.size() == 3");;
        const _: () = assert!(true, "u8\"ABC\"sv.size() == 3");;
        const _: () = assert!(true, "L\"ABC\"sv.size() == 3");;
        const _: () = assert!(true, "u\"ABC\"sv.size() == 3");;
        const _: () = assert!(true, "U\"ABC\"sv.size() == 3");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let foo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 0_()));
    }
    {
        let foo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 0_()));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
