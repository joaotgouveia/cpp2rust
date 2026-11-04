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
pub struct test_macros_detail_is_same_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t____std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t____std_basic_string_char8_t__std_char_traits_char8_t___std_allocator_char8_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t____std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t____std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t____std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t____std_basic_string_char16_t__std_char_traits_char16_t___std_allocator_char16_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t____std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___
{}
impl CppMove for test_macros_detail_is_same_std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t____std_basic_string_char32_t__std_char_traits_char32_t___std_allocator_char32_t___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
        let foo: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
        let Lfoo: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
        let u8foo: Value<Vec<u8>> = Rc::new(RefCell::new(vec![u8::default()]));
        let ufoo: Value<Vec<u16>> = Rc::new(RefCell::new(vec![u16::default()]));
        let Ufoo: Value<Vec<u32>> = Rc::new(RefCell::new(vec![u32::default()]));
        foo.as_pointer().write(unimplemented!());
        assert!(({ (((*foo.borrow()).len() - 1) == 0_usize) }));
        u8foo.as_pointer().write(unimplemented!());
        assert!(({ (((*u8foo.borrow()).len() - 1) == 0_usize) }));
        Lfoo.as_pointer().write(unimplemented!());
        assert!(({ (((*Lfoo.borrow()).len() - 1) == 0_usize) }));
        ufoo.as_pointer().write(unimplemented!());
        assert!(({ (((*ufoo.borrow()).len() - 1) == 0_usize) }));
        Ufoo.as_pointer().write(unimplemented!());
        assert!(({ (((*Ufoo.borrow()).len() - 1) == 0_usize) }));
        foo.as_pointer().write(unimplemented!());
        assert!(({ (((*foo.borrow()).len() - 1) == 1_usize) }));
        u8foo.as_pointer().write(unimplemented!());
        assert!(({ (((*u8foo.borrow()).len() - 1) == 1_usize) }));
        Lfoo.as_pointer().write(unimplemented!());
        assert!(({ (((*Lfoo.borrow()).len() - 1) == 1_usize) }));
        ufoo.as_pointer().write(unimplemented!());
        assert!(({ (((*ufoo.borrow()).len() - 1) == 1_usize) }));
        Ufoo.as_pointer().write(unimplemented!());
        assert!(({ (((*Ufoo.borrow()).len() - 1) == 1_usize) }));
        foo.as_pointer().write(unimplemented!());
        assert!(
            ({
                Ptr::<i8>::from_string_literal(b"ABC").with_c_str(|__s| {
                    (*foo.borrow())[..(*foo.borrow()).len().saturating_sub(1)] == *__s
                })
            })
        );
        assert!(unimplemented!());
        u8foo.as_pointer().write(unimplemented!());
        assert!(
            ({
                Ptr::<u8>::from_string_literal(&[65 as u8, 66 as u8, 67 as u8, 0 as u8]).with_c_str(
                    |__s| (*u8foo.borrow())[..(*u8foo.borrow()).len().saturating_sub(1)] == *__s,
                )
            })
        );
        assert!(unimplemented!());
        Lfoo.as_pointer().write(unimplemented!());
        assert!(
            ({
                Ptr::<i32>::from_string_literal(&[65 as i32, 66 as i32, 67 as i32, 0 as i32])
                    .with_c_str(|__s| {
                        (*Lfoo.borrow())[..(*Lfoo.borrow()).len().saturating_sub(1)] == *__s
                    })
            })
        );
        assert!(unimplemented!());
        ufoo.as_pointer().write(unimplemented!());
        assert!(
            ({
                Ptr::<u16>::from_string_literal(&[65 as u16, 66 as u16, 67 as u16, 0 as u16])
                    .with_c_str(|__s| {
                        (*ufoo.borrow())[..(*ufoo.borrow()).len().saturating_sub(1)] == *__s
                    })
            })
        );
        assert!(unimplemented!());
        Ufoo.as_pointer().write(unimplemented!());
        assert!(
            ({
                Ptr::<u32>::from_string_literal(&[65 as u32, 66 as u32, 67 as u32, 0 as u32])
                    .with_c_str(|__s| {
                        (*Ufoo.borrow())[..(*Ufoo.borrow()).len().saturating_sub(1)] == *__s
                    })
            })
        );
        assert!(unimplemented!());
    }
    {
        let mut foo: Vec<i8> = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut foo: Vec<i8> = unimplemented!();
        assert!(unimplemented!());
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
