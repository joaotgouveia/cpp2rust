extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub fn for_each_5(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_,
    f: FnPtr<Generic>,
) {
    let f: Value<FnPtr<Generic>> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ (*f.borrow()).spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            ({ (*f.borrow()).spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            ({ (*f.borrow()).spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            ({ (*f.borrow()).spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            ({ (*f.borrow()).spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            ({ (*f.borrow()).spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            ({ (*f.borrow()).spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            ({ (*f.borrow()).spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            ({ (*f.borrow()).spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            ({ (*f.borrow()).spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            ({ (*f.borrow()).spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            ({ (*f.borrow()).spec::<fn()>(11).call() });
            0_()
        };
        let _arg12: () = {
            ({ (*f.borrow()).spec::<fn()>(12).call() });
            0_()
        };
        let _arg13: () = {
            ({ (*f.borrow()).spec::<fn()>(13).call() });
            0_()
        };
        let _arg14: () = {
            ({ (*f.borrow()).spec::<fn()>(14).call() });
            0_()
        };
        let _arg15: () = {
            ({ (*f.borrow()).spec::<fn()>(15).call() });
            0_()
        };
        let _arg16: () = {
            ({ (*f.borrow()).spec::<fn()>(16).call() });
            0_()
        };
        swallow_6(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16,
        )
    });
}
pub fn swallow_6(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
    mut _a7: (),
    mut _a8: (),
    mut _a9: (),
    mut _a10: (),
    mut _a11: (),
    mut _a12: (),
    mut _a13: (),
    mut _a14: (),
    mut _a15: (),
    mut _a16: (),
) {
}
pub fn test_appending_7(k: (), N: (), new_capacity: ()) {
    let k: Value<()> = Rc::new(RefCell::new(k));
    let N: Value<()> = Rc::new(RefCell::new(N));
    let new_capacity: Value<()> = Rc::new(RefCell::new(new_capacity));
    unimplemented!();
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_appending_8(k: (), N: (), new_capacity: ()) {
    let k: Value<()> = Rc::new(RefCell::new(k));
    let N: Value<()> = Rc::new(RefCell::new(N));
    let new_capacity: Value<()> = Rc::new(RefCell::new(new_capacity));
    unimplemented!();
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_appending_9(k: (), N: (), new_capacity: ()) {
    let k: Value<()> = Rc::new(RefCell::new(k));
    let N: Value<()> = Rc::new(RefCell::new(N));
    let new_capacity: Value<()> = Rc::new(RefCell::new(new_capacity));
    unimplemented!();
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_appending_10(k: (), N: (), new_capacity: ()) {
    let k: Value<()> = Rc::new(RefCell::new(k));
    let N: Value<()> = Rc::new(RefCell::new(N));
    let new_capacity: Value<()> = Rc::new(RefCell::new(new_capacity));
    unimplemented!();
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_appending_11(k: (), N: (), new_capacity: ()) {
    let k: Value<()> = Rc::new(RefCell::new(k));
    let N: Value<()> = Rc::new(RefCell::new(N));
    let new_capacity: Value<()> = Rc::new(RefCell::new(new_capacity));
    unimplemented!();
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_truncating_12(mut o: (), N: ()) {
    let N: Value<()> = Rc::new(RefCell::new(N));
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_truncating_13(mut o: (), N: ()) {
    let N: Value<()> = Rc::new(RefCell::new(N));
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_truncating_14(mut o: (), N: ()) {
    let N: Value<()> = Rc::new(RefCell::new(N));
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_truncating_15(mut o: (), N: ()) {
    let N: Value<()> = Rc::new(RefCell::new(N));
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_truncating_16(mut o: (), N: ()) {
    let N: Value<()> = Rc::new(RefCell::new(N));
    unimplemented!();
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_17() -> () {
    ({ test_appending_7(10_(), 15_(), 15_()) });
    ({ test_appending_7(10_(), 15_(), 20_()) });
    ({ test_appending_7(10_(), 40_(), 40_()) });
    ({ test_appending_7(10_(), 40_(), 50_()) });
    ({ test_appending_7(30_(), 35_(), 35_()) });
    ({ test_appending_7(30_(), 35_(), 45_()) });
    ({ test_appending_7(10_(), 15_(), 30_()) });
    ({ test_truncating_12(15_(), 10_()) });
    ({ test_truncating_12(40_(), 35_()) });
    ({ test_truncating_12(40_(), 10_()) });
    return true;
}
pub fn test_18() -> () {
    ({ test_appending_8(10_(), 15_(), 15_()) });
    ({ test_appending_8(10_(), 15_(), 20_()) });
    ({ test_appending_8(10_(), 40_(), 40_()) });
    ({ test_appending_8(10_(), 40_(), 50_()) });
    ({ test_appending_8(30_(), 35_(), 35_()) });
    ({ test_appending_8(30_(), 35_(), 45_()) });
    ({ test_appending_8(10_(), 15_(), 30_()) });
    ({ test_truncating_13(15_(), 10_()) });
    ({ test_truncating_13(40_(), 35_()) });
    ({ test_truncating_13(40_(), 10_()) });
    return true;
}
pub fn test_19() -> () {
    ({ test_appending_9(10_(), 15_(), 15_()) });
    ({ test_appending_9(10_(), 15_(), 20_()) });
    ({ test_appending_9(10_(), 40_(), 40_()) });
    ({ test_appending_9(10_(), 40_(), 50_()) });
    ({ test_appending_9(30_(), 35_(), 35_()) });
    ({ test_appending_9(30_(), 35_(), 45_()) });
    ({ test_appending_9(10_(), 15_(), 30_()) });
    ({ test_truncating_14(15_(), 10_()) });
    ({ test_truncating_14(40_(), 35_()) });
    ({ test_truncating_14(40_(), 10_()) });
    return true;
}
pub fn test_20() -> () {
    ({ test_appending_10(10_(), 15_(), 15_()) });
    ({ test_appending_10(10_(), 15_(), 20_()) });
    ({ test_appending_10(10_(), 40_(), 40_()) });
    ({ test_appending_10(10_(), 40_(), 50_()) });
    ({ test_appending_10(30_(), 35_(), 35_()) });
    ({ test_appending_10(30_(), 35_(), 45_()) });
    ({ test_appending_10(10_(), 15_(), 30_()) });
    ({ test_truncating_15(15_(), 10_()) });
    ({ test_truncating_15(40_(), 35_()) });
    ({ test_truncating_15(40_(), 10_()) });
    return true;
}
pub fn test_21() -> () {
    ({ test_appending_11(10_(), 15_(), 15_()) });
    ({ test_appending_11(10_(), 15_(), 20_()) });
    ({ test_appending_11(10_(), 40_(), 40_()) });
    ({ test_appending_11(10_(), 40_(), 50_()) });
    ({ test_appending_11(30_(), 35_(), 35_()) });
    ({ test_appending_11(30_(), 35_(), 45_()) });
    ({ test_appending_11(10_(), 15_(), 30_()) });
    ({ test_truncating_16(15_(), 10_()) });
    ({ test_truncating_16(40_(), 35_()) });
    ({ test_truncating_16(40_(), 10_()) });
    return true;
}
pub fn test_value_categories_22() -> () {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RefQualified_23 {}
pub fn test_integer_like_return_types_24() -> () {
    ({
        for_each_5 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_  >::default()  , lambda! ( Generic , { } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : ()  = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } )  , )
    });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_17() });
    ({ test_18() });
    ({ test_19() });
    ({ test_20() });
    const _: () = assert!(
        true,
        "test<std::basic_string<char, std::char_traits<char>, std::allocator<char>>>()"
    );;
    const _: () = assert!(
        true,
        "test<std::basic_string<char8_t, std::char_traits<char8_t>, std::allocator<char8_t>>>()"
    );;
    const _: () = assert!(
        true,
        "test<std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>>()"
    );;
    const _: () = assert!(
        true,
        "test<std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>>()"
    );;
    ({ test_21() });
    const _: () = assert!(
        true,
        "test<std::basic_string<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>>()"
    );;
    ({ test_value_categories_22() });
    ({ test_integer_like_return_types_24() });
    const _: () = assert!(true, "test_value_categories()");;
    const _: () = assert!(true, "test_integer_like_return_types()");;
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub trait RefQualified_23Impl {
    fn operator_call(&self, _a0: Ptr<()>, _a1: ()) -> ();
}
impl RefQualified_23Impl for Ptr<RefQualified_23> {
    fn operator_call(&self, mut _a0: Ptr<()>, mut _a1: ()) -> () {
        return 0_();
    }
}
pub fn __cpp2rust_init_globals() {}
