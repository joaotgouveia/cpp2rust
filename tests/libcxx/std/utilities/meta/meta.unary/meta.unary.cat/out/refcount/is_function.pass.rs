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
pub struct test_is_function_void____ {}
impl CppMove for test_is_function_void____ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_function_int__int__ {}
impl CppMove for test_is_function_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_function_int__int__double__ {}
impl CppMove for test_is_function_int__int__double__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_function_int__Abstract_ptr__ {}
impl CppMove for test_is_function_int__Abstract_ptr__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_function_void_______ {}
impl CppMove for test_is_function_void_______ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_function_void____noexcept_ {}
impl CppMove for test_is_function_void____noexcept_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_function_void____const_refref_noexcept_ {}
impl CppMove for test_is_function_void____const_refref_noexcept_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_std_nullptr_t_ {}
impl CppMove for test_is_not_function_std_nullptr_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_void_ {}
impl CppMove for test_is_not_function_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_int_ {}
impl CppMove for test_is_not_function_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_int_ref_ {}
impl CppMove for test_is_not_function_int_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_int_refref_ {}
impl CppMove for test_is_not_function_int_refref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_int_ptr_ {}
impl CppMove for test_is_not_function_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_double_ {}
impl CppMove for test_is_not_function_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_chararr3arr_ {}
impl CppMove for test_is_not_function_chararr3arr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_chararrarr_ {}
impl CppMove for test_is_not_function_chararrarr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_Union_ {}
impl CppMove for test_is_not_function_Union_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_Enum_ {}
impl CppMove for test_is_not_function_Enum_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_void__ptr____ {}
impl CppMove for test_is_not_function_void__ptr____ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_Empty_ {}
impl CppMove for test_is_not_function_Empty_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_bit_zero_ {}
impl CppMove for test_is_not_function_bit_zero_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_NotEmpty_ {}
impl CppMove for test_is_not_function_NotEmpty_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_Abstract_ {}
impl CppMove for test_is_not_function_Abstract_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_Abstract_ptr_ {}
impl CppMove for test_is_not_function_Abstract_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_is_not_function_incomplete_type_ {}
impl CppMove for test_is_not_function_incomplete_type_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
impl CppMove for Empty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct NotEmpty {}
impl CppMove for NotEmpty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(1)]
pub struct Union {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Box<[u8]>,
}
impl Union {}
impl CppMove for Union {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Union {
    fn default() -> Self {
        Union {
            __bytes: Box::from([0u8; 1]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bit_zero {
    #[offset(0)]
    pub anon_0: i32,
}
impl CppMove for bit_zero {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type Enum = u32;
pub const Enum_zero: Enum = 0;
pub const Enum_one: Enum = 1;
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
    <test_is_function_void____>::default();
    <test_is_function_int__int__>::default();
    <test_is_function_int__int__double__>::default();
    <test_is_function_int__Abstract_ptr__>::default();
    <test_is_function_void_______>::default();
    <test_is_not_function_std_nullptr_t_>::default();
    <test_is_not_function_void_>::default();
    <test_is_not_function_int_>::default();
    <test_is_not_function_int_ref_>::default();
    <test_is_not_function_int_refref_>::default();
    <test_is_not_function_int_ptr_>::default();
    <test_is_not_function_double_>::default();
    <test_is_not_function_chararr3arr_>::default();
    <test_is_not_function_chararrarr_>::default();
    <test_is_not_function_Union_>::default();
    <test_is_not_function_Enum_>::default();
    <test_is_not_function_void__ptr____>::default();
    <test_is_not_function_Empty_>::default();
    <test_is_not_function_bit_zero_>::default();
    <test_is_not_function_NotEmpty_>::default();
    <test_is_not_function_Abstract_>::default();
    <test_is_not_function_Abstract_ptr_>::default();
    <test_is_not_function_incomplete_type_>::default();
    <test_is_function_void____noexcept_>::default();
    <test_is_function_void____const_refref_noexcept_>::default();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
