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
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
impl CppMove for test_macros_detail_is_same_char_ptr__char_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_ptr__long_ptr_ {}
impl CppMove for test_macros_detail_is_same_long_ptr__long_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double_ptr__double_ptr_ {}
impl CppMove for test_macros_detail_is_same_double_ptr__double_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_double_ptr__long_double_ptr_ {}
impl CppMove for test_macros_detail_is_same_long_double_ptr__long_double_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S_ptr__S_ptr_ {}
impl CppMove for test_macros_detail_is_same_S_ptr__S_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S4_ptr__S4_ptr_ {}
impl CppMove for test_macros_detail_is_same_S4_ptr__S4_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S8_ptr__S8_ptr_ {}
impl CppMove for test_macros_detail_is_same_S8_ptr__S8_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_8 = u32;
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S16_ptr__S16_ptr_ {}
impl CppMove for test_macros_detail_is_same_S16_ptr__S16_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_9 = u32;
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S32_ptr__S32_ptr_ {}
impl CppMove for test_macros_detail_is_same_S32_ptr__S32_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_10 = u32;
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S64_ptr__S64_ptr_ {}
impl CppMove for test_macros_detail_is_same_S64_ptr__S64_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_11 = u32;
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S128_ptr__S128_ptr_ {}
impl CppMove for test_macros_detail_is_same_S128_ptr__S128_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn check_12(mut p: Ptr<i8>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 1_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_13((p).clone(),) }) }));
    }
    {}
    {}
    {}
    {}
    {}
    {}
    {}
}
pub fn check_14(mut p: Ptr<i32>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 4_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_15((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_16((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_17((p).clone(),) }) }));
    }
    {}
    {}
    {}
    {}
    {}
}
pub fn check_18(mut p: Ptr<i64>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 8_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_19((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_20((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_21((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_22((p).clone(),) }) }));
    }
    {}
    {}
    {}
    {}
}
pub fn check_23(mut p: Ptr<f64>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 8_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_24((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_25((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_26((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_27((p).clone(),) }) }));
    }
    {}
    {}
    {}
    {}
}
pub fn check_28(mut p: Ptr<f64>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 16_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_29((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_30((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_31((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_32((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_33((p).clone(),) }) }));
    }
    {}
    {}
    {}
}
pub fn check_34(mut p: Ptr<S>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 1_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_35((p).clone(),) }) }));
    }
    {}
    {}
    {}
    {}
    {}
    {}
    {}
}
pub fn check_36(mut p: Ptr<S4>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 4_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_37((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_38((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_39((p).clone(),) }) }));
    }
    {}
    {}
    {}
    {}
    {}
}
pub fn check_40(mut p: Ptr<S8>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 8_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_41((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_42((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_43((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_44((p).clone(),) }) }));
    }
    {}
    {}
    {}
    {}
}
pub fn check_45(mut p: Ptr<S16>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 16_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_46((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_47((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_48((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_49((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_50((p).clone(),) }) }));
    }
    {}
    {}
    {}
}
pub fn check_51(mut p: Ptr<S32>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 32_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_52((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_53((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_54((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_55((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_56((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_57((p).clone(),) }) }));
    }
    {}
    {}
}
pub fn check_58(mut p: Ptr<S64>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 64_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_59((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_60((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_61((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_62((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_63((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_64((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_65((p).clone(),) }) }));
    }
    {}
}
pub fn check_66(mut p: Ptr<S128>) {
    const _: () = assert!(true, "");;
    let mut alignment: usize = 128_usize;
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_67((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_68((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_69((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_70((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_71((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_72((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_73((p).clone(),) }) }));
    }
    {
        assert!(({ (p).clone() } == { ({ assume_aligned_74((p).clone(),) }) }));
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S4 {}
impl CppMove for S4 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S8 {}
impl CppMove for S8 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S16 {}
impl CppMove for S16 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct S32 {}
impl CppMove for S32 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(64)]
pub struct S64 {}
impl CppMove for S64 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(128)]
pub struct S128 {}
impl CppMove for S128 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn tests_75() -> bool {
    let c: Value<i8> = Rc::new(RefCell::new(0_i8));
    let i: Value<i32> = Rc::new(RefCell::new(0_i32));
    let l: Value<i64> = Rc::new(RefCell::new(0_i64));
    let d: Value<f64> = Rc::new(RefCell::new(0_f64));
    let ld: Value<f64> = Rc::new(RefCell::new(0_f64));
    ({ check_12((c.as_pointer())) });
    ({ check_14((i.as_pointer())) });
    ({ check_18((l.as_pointer())) });
    ({ check_23((d.as_pointer())) });
    ({ check_28((ld.as_pointer())) });
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    let s4: Value<S4> = Rc::new(RefCell::new(<S4>::default()));
    let s8: Value<S8> = Rc::new(RefCell::new(<S8>::default()));
    let s16: Value<S16> = Rc::new(RefCell::new(<S16>::default()));
    let s32: Value<S32> = Rc::new(RefCell::new(<S32>::default()));
    let s64: Value<S64> = Rc::new(RefCell::new(<S64>::default()));
    let s128: Value<S128> = Rc::new(RefCell::new(<S128>::default()));
    ({ check_34((s.as_pointer())) });
    ({ check_36((s4.as_pointer())) });
    ({ check_40((s8.as_pointer())) });
    ({ check_45((s16.as_pointer())) });
    ({ check_51((s32.as_pointer())) });
    ({ check_58((s64.as_pointer())) });
    ({ check_66((s128.as_pointer())) });
    return true;
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
    ({ tests_75() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
