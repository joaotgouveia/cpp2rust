extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
pub unsafe fn is_string_asan_correct_0(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_2(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_3(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_4(_a0: *const ()) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub unsafe fn for_each_5(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            (unsafe { f.spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            (unsafe { f.spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            (unsafe { f.spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            (unsafe { f.spec::<fn()>(11).call() });
            0_()
        };
        let _arg12: () = {
            (unsafe { f.spec::<fn()>(12).call() });
            0_()
        };
        let _arg13: () = {
            (unsafe { f.spec::<fn()>(13).call() });
            0_()
        };
        let _arg14: () = {
            (unsafe { f.spec::<fn()>(14).call() });
            0_()
        };
        let _arg15: () = {
            (unsafe { f.spec::<fn()>(15).call() });
            0_()
        };
        let _arg16: () = {
            (unsafe { f.spec::<fn()>(16).call() });
            0_()
        };
        swallow_6(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16,
        )
    });
}
pub unsafe fn swallow_6(
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
pub unsafe fn test_appending_7(mut k: (), mut N: (), mut new_capacity: ()) {
    unimplemented!();
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_appending_8(mut k: (), mut N: (), mut new_capacity: ()) {
    unimplemented!();
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_appending_9(mut k: (), mut N: (), mut new_capacity: ()) {
    unimplemented!();
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_appending_10(mut k: (), mut N: (), mut new_capacity: ()) {
    unimplemented!();
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_appending_11(mut k: (), mut N: (), mut new_capacity: ()) {
    unimplemented!();
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_truncating_12(mut o: (), mut N: ()) {
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_truncating_13(mut o: (), mut N: ()) {
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_truncating_14(mut o: (), mut N: ()) {
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_truncating_15(mut o: (), mut N: ()) {
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_truncating_16(mut o: (), mut N: ()) {
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    let expected: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_17() -> () {
    (unsafe { test_appending_7(10_(), 15_(), 15_()) });
    (unsafe { test_appending_7(10_(), 15_(), 20_()) });
    (unsafe { test_appending_7(10_(), 40_(), 40_()) });
    (unsafe { test_appending_7(10_(), 40_(), 50_()) });
    (unsafe { test_appending_7(30_(), 35_(), 35_()) });
    (unsafe { test_appending_7(30_(), 35_(), 45_()) });
    (unsafe { test_appending_7(10_(), 15_(), 30_()) });
    (unsafe { test_truncating_12(15_(), 10_()) });
    (unsafe { test_truncating_12(40_(), 35_()) });
    (unsafe { test_truncating_12(40_(), 10_()) });
    return true;
}
pub unsafe fn test_18() -> () {
    (unsafe { test_appending_8(10_(), 15_(), 15_()) });
    (unsafe { test_appending_8(10_(), 15_(), 20_()) });
    (unsafe { test_appending_8(10_(), 40_(), 40_()) });
    (unsafe { test_appending_8(10_(), 40_(), 50_()) });
    (unsafe { test_appending_8(30_(), 35_(), 35_()) });
    (unsafe { test_appending_8(30_(), 35_(), 45_()) });
    (unsafe { test_appending_8(10_(), 15_(), 30_()) });
    (unsafe { test_truncating_13(15_(), 10_()) });
    (unsafe { test_truncating_13(40_(), 35_()) });
    (unsafe { test_truncating_13(40_(), 10_()) });
    return true;
}
pub unsafe fn test_19() -> () {
    (unsafe { test_appending_9(10_(), 15_(), 15_()) });
    (unsafe { test_appending_9(10_(), 15_(), 20_()) });
    (unsafe { test_appending_9(10_(), 40_(), 40_()) });
    (unsafe { test_appending_9(10_(), 40_(), 50_()) });
    (unsafe { test_appending_9(30_(), 35_(), 35_()) });
    (unsafe { test_appending_9(30_(), 35_(), 45_()) });
    (unsafe { test_appending_9(10_(), 15_(), 30_()) });
    (unsafe { test_truncating_14(15_(), 10_()) });
    (unsafe { test_truncating_14(40_(), 35_()) });
    (unsafe { test_truncating_14(40_(), 10_()) });
    return true;
}
pub unsafe fn test_20() -> () {
    (unsafe { test_appending_10(10_(), 15_(), 15_()) });
    (unsafe { test_appending_10(10_(), 15_(), 20_()) });
    (unsafe { test_appending_10(10_(), 40_(), 40_()) });
    (unsafe { test_appending_10(10_(), 40_(), 50_()) });
    (unsafe { test_appending_10(30_(), 35_(), 35_()) });
    (unsafe { test_appending_10(30_(), 35_(), 45_()) });
    (unsafe { test_appending_10(10_(), 15_(), 30_()) });
    (unsafe { test_truncating_15(15_(), 10_()) });
    (unsafe { test_truncating_15(40_(), 35_()) });
    (unsafe { test_truncating_15(40_(), 10_()) });
    return true;
}
pub unsafe fn test_21() -> () {
    (unsafe { test_appending_11(10_(), 15_(), 15_()) });
    (unsafe { test_appending_11(10_(), 15_(), 20_()) });
    (unsafe { test_appending_11(10_(), 40_(), 40_()) });
    (unsafe { test_appending_11(10_(), 40_(), 50_()) });
    (unsafe { test_appending_11(30_(), 35_(), 35_()) });
    (unsafe { test_appending_11(30_(), 35_(), 45_()) });
    (unsafe { test_appending_11(10_(), 15_(), 30_()) });
    (unsafe { test_truncating_16(15_(), 10_()) });
    (unsafe { test_truncating_16(40_(), 35_()) });
    (unsafe { test_truncating_16(40_(), 10_()) });
    return true;
}
pub unsafe fn test_value_categories_22() -> () {
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefQualified_23 {}
impl RefQualified_23 {
    pub unsafe fn operator_call(&mut self, mut _a0: *mut (), mut _a1: ()) -> () {
        return 0_();
    }
}
pub unsafe fn test_integer_like_return_types_24() -> () {
    (unsafe {
        for_each_5 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } , | | { let mut s : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } ) , )
    });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_17() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    (unsafe { test_20() });
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
    (unsafe { test_21() });
    const _: () = assert!(
        true,
        "test<std::basic_string<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>>()"
    );;
    (unsafe { test_value_categories_22() });
    (unsafe { test_integer_like_return_types_24() });
    const _: () = assert!(true, "test_value_categories()");;
    const _: () = assert!(true, "test_integer_like_return_types()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
