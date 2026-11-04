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
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double___types_type_list_Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool__float__double__long_double___types_type_list_Enum__EnumClass__void_ptr__const_void_ptr__Class_ptr__std_nullptr_t__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type Enum = u32;
pub type EnumClass = bool;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
impl CppMove for Class {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestHashEnabled {}
impl CppMove for TestHashEnabled {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct B {}
impl CppMove for B {
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
    let mut nullopt_hash: usize = ({
        let ___t: Value<std_optional_double_> =
            Rc::new(RefCell::new(<std_optional_double_>::default()));
        (<std_hash_std_optional_double__>::default() as std___optional_hash_double__double_)(
            ___t.as_pointer(),
        )
    });
    {
        let opt: Value<std_optional_B_> = Rc::new(RefCell::new(<std_optional_B_>::default()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let opt: Value<std_optional_int_> = Rc::new(RefCell::new(<std_optional_int_>::default()));
        assert!(
            (({
                (<std_hash_std_optional_int__>::default() as std___optional_hash_int__int_)(
                    opt.as_pointer(),
                )
            }) == nullopt_hash)
        );
        (*opt.borrow_mut()) = 2;
        assert!(
            (({
                (<std_hash_std_optional_int__>::default() as std___optional_hash_int__int_)(
                    opt.as_pointer(),
                )
            }) == ({ <std_hash_int_>::default()(((*opt.borrow()).read()),) }))
        );
    }
    {
        let opt : Value<std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(<std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char___  >::default() )) ;
        assert!(
            (({
                ( <std_hash_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() as std___optional_hash_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char___ ) ( opt .as_pointer()  , )
            }) == nullopt_hash)
        );
        (*opt.borrow_mut()) = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"123").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        assert!(
            (({
                ( <std_hash_std_optional_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() as std___optional_hash_std_basic_string_char__std_char_traits_char___std_allocator_char____std_basic_string_char__std_char_traits_char___std_allocator_char___ ) ( opt .as_pointer()  , )
            }) == ({
                ( <std_hash_std_basic_string_char__std_char_traits_char___std_allocator_char___  >::default() as std___str_hash_base_char__std_allocator_char___std_basic_string_char__std_char_traits_char___std_allocator_char___ ) ( opt .as_pointer()  , )
            }))
        );
    }
    {
        let opt: Value<std_optional_std_unique_ptr_int__std_default_delete_int___> = Rc::new(
            RefCell::new(<std_optional_std_unique_ptr_int__std_default_delete_int___>::default()),
        );
        assert!(
            (({
                ( <std_hash_std_optional_std_unique_ptr_int__std_default_delete_int____  >::default() as std___optional_hash_std_unique_ptr_int__std_default_delete_int____std_unique_ptr_int__std_default_delete_int___ ) ( opt .as_pointer()  , )
            }) == nullopt_hash)
        );
        (*opt.borrow_mut()) = Ptr::alloc(3).to_owned_opt();
        assert!(
            (({
                ( <std_hash_std_optional_std_unique_ptr_int__std_default_delete_int____  >::default() as std___optional_hash_std_unique_ptr_int__std_default_delete_int____std_unique_ptr_int__std_default_delete_int___ ) ( opt .as_pointer()  , )
            }) == ({
                (<std_hash_std_unique_ptr_int__std_default_delete_int___>::default()
                    as std___uniq_ptr_hash_std_unique_ptr_int__std_default_delete_int____int_ptr_)(
                    opt.as_pointer(),
                )
            }))
        );
    }
    {
        ({ test_hash_enabled_0(None) });
        ({ test_hash_enabled_1(None) });
        ({ test_hash_enabled_2(None) });
        ({ test_hash_enabled_3(None) });
        ({ test_hash_disabled_4() });
        ({ test_hash_disabled_5() });
        ({ test_hash_enabled_6(None) });
        ({ test_hash_enabled_7(None) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
